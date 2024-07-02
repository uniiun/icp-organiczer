<template>
    <div class="p-4 bg-white rounded shadow-md">
        <h2 class="text-2xl font-bold mb-4 text-center">Calendar</h2>
        <div class="flex justify-between items-center mb-2">
            <button @click="previousMonth" class="bg-gray-200 text-gray-800 px-2 py-1 rounded">❮</button>
            <span class="text-lg font-semibold">{{ currentMonthName }} {{ currentYear }}</span>
            <button @click="nextMonth" class="bg-gray-200 text-gray-800 px-2 py-1 rounded">❯</button>
        </div>
        <div class="grid grid-cols-7 gap-1 text-center font-bold">
            <div v-for="day in daysOfWeek" :key="day">{{ day }}</div>
        </div>
        <div class="grid grid-cols-7 gap-1 text-center">
            <div v-for="blank in blanks" :key="'blank' + blank" class="p-2"></div>
            <div v-for="(day, index) in daysInMonth" :key="index" class="relative">
                <span @click="selectDate(day)" :class="getDayClass(day)"
                    class="p-2 cursor-pointer rounded-full hover:bg-gray-200 inline-block w-full">{{ day }}</span>
                <span v-if="hasEvent(day)" class="absolute top-1 right-1 w-2 h-2 bg-blue-500 rounded-full"></span>
            </div>
        </div>
        <div v-if="selectedDate" class="mt-4">
            <h3 class="text-xl font-semibold mb-2">Add Event on {{ selectedDate }}</h3>
            <form @submit.prevent="saveEvent" class="space-y-2">
                <input v-model="newEvent.title" placeholder="Title" required class="w-full p-2 border rounded" />
                <input v-model="newEvent.description" placeholder="Description" class="w-full p-2 border rounded" />
                <input v-model="newEvent.time" type="time" class="w-full p-2 border rounded" />
                <button type="submit" class="bg-green-500 text-white px-4 py-2 rounded">Save Event</button>
            </form>
        </div>
        <h3 class="text-xl font-semibold mt-6">Upcoming Events</h3>
        <ul class="list-disc pl-5 mt-2">
            <li v-for="(event, index) in futureEvents" :key="index">
                <strong>{{ event.date }} {{ event.time }}</strong>: {{ event.title }} -
                {{ event.description || 'No description' }}
                <button @click="editEvent(index, true)"
                    class="bg-yellow-500 text-white px-2 py-1 rounded ml-2">Edit</button>
                <button @click="deleteEvent(index)" class="bg-red-500 text-white px-2 py-1 rounded ml-2">Delete</button>
            </li>
        </ul>
        <h3 class="text-xl font-semibold mt-6">All Events</h3>
        <ul class="list-disc pl-5 mt-2">
            <li v-for="(event, index) in allEvents" :key="'all-' + index">
                <strong>{{ event.date }} {{ event.time }}</strong>: {{ event.title }} -
                {{ event.description || 'No description' }}
                <button @click="editEvent(index, false)"
                    class="bg-yellow-500 text-white px-2 py-1 rounded ml-2">Edit</button>
                <button @click="deleteEvent(index)" class="bg-red-500 text-white px-2 py-1 rounded ml-2">Delete</button>
            </li>
        </ul>
    </div>
</template>

<script setup>
    import {
        ref,
        onMounted,
        computed
    } from 'vue';
    import {
        getAllEvents,
        createEvent,
        updateEvent,
        deleteEvent as removeEvent
    } from '../api/Calendar.js';

    const currentYear = ref(new Date().getFullYear());
    const currentMonth = ref(new Date().getMonth());
    const daysOfWeek = ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa'];
    const daysInMonth = ref([]);
    const blanks = ref([]);
    const selectedDate = ref(null);
    const newEvent = ref({
        title: '',
        description: '',
        time: ''
    });
    const events = ref([]);
    const editingEventIndex = ref(null);
    const editingFutureEvent = ref(true);

    const currentMonthName = computed(() => {
        return new Date(currentYear.value, currentMonth.value).toLocaleString('default', {
            month: 'long'
        });
    });

    const futureEvents = computed(() => {
        const today = new Date().toISOString().split('T')[0];
        const nextMonth = new Date();
        nextMonth.setMonth(nextMonth.getMonth() + 1);
        const nextMonthDate = nextMonth.toISOString().split('T')[0];
        return events.value.filter(event => event.date >= today && event.date <= nextMonthDate);
    });

    const allEvents = computed(() => {
        return events.value;
    });

    async function fetchEvents() {
        events.value = await getAllEvents();
    }

    function previousMonth() {
        currentMonth.value--;
        if (currentMonth.value < 0) {
            currentMonth.value = 11;
            currentYear.value--;
        }
        calculateDaysInMonth();
    }

    function nextMonth() {
        currentMonth.value++;
        if (currentMonth.value > 11) {
            currentMonth.value = 0;
            currentYear.value++;
        }
        calculateDaysInMonth();
    }

    function calculateDaysInMonth() {
        const days = new Date(currentYear.value, currentMonth.value + 1, 0).getDate();
        const firstDay = new Date(currentYear.value, currentMonth.value, 1).getDay();
        blanks.value = Array.from({
            length: firstDay
        }, (_, i) => i);
        daysInMonth.value = Array.from({
            length: days
        }, (_, i) => i + 1);
    }

    function selectDate(day) {
        selectedDate.value =
            `${currentYear.value}-${String(currentMonth.value + 1).padStart(2, '0')}-${String(day).padStart(2, '0')}`;
        newEvent.value = {
            title: '',
            description: '',
            time: ''
        };
        editingEventIndex.value = null;
    }

    function getDayClass(day) {
        const date =
            `${currentYear.value}-${String(currentMonth.value + 1).padStart(2, '0')}-${String(day).padStart(2, '0')}`;
        if (selectedDate.value === date) {
            return 'bg-blue-500 text-white';
        } else if (hasEvent(day)) {
            return 'border border-blue-500';
        } else {
            return '';
        }
    }

    function hasEvent(day) {
        const date =
            `${currentYear.value}-${String(currentMonth.value + 1).padStart(2, '0')}-${String(day).padStart(2, '0')}`;
        return events.value.some(event => event.date === date);
    }

    function saveEvent() {
        if (editingEventIndex.value !== null) {
            const index = editingEventIndex.value;
            const eventsArray = editingFutureEvent.value ? futureEvents.value : allEvents.value;
            updateEvent(eventsArray[index].id, newEvent.value.title, newEvent.value.description, selectedDate.value,
                    newEvent.value.time)
                .then(() => {
                    fetchEvents();
                });
        } else {
            createEvent(newEvent.value.title, newEvent.value.description, selectedDate.value, newEvent.value.time)
                .then(() => {
                    fetchEvents();
                });
        }
        selectedDate.value = null;
        newEvent.value = {
            title: '',
            description: '',
            time: ''
        };
    }

    function editEvent(index, isFutureEvent) {
        const event = isFutureEvent ? futureEvents.value[index] : allEvents.value[index];
        selectedDate.value = event.date;
        newEvent.value = {
            title: event.title,
            description: event.description,
            time: event.time
        };
        editingEventIndex.value = index;
        editingFutureEvent.value = isFutureEvent;
    }

    function deleteEvent(index) {
        const eventsArray = editingFutureEvent.value ? futureEvents.value : allEvents.value;
        removeEvent(eventsArray[index].id)
            .then(() => {
                fetchEvents();
            });
    }

    onMounted(() => {
        calculateDaysInMonth();
        fetchEvents();
    });
</script>

<style scoped>
    .calendar {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>