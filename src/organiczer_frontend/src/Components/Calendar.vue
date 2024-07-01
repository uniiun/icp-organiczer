<template>
    <div class="p-4">
        <h2 class="text-2xl font-bold mb-4">Calendar</h2>
        <div class="calendar">
            <div class="flex justify-between items-center mb-2">
                <button @click="previousMonth" class="bg-blue-500 text-white px-2 py-1 rounded">❮</button>
                <span class="text-lg font-semibold">{{ currentMonthName }} {{ currentYear }}</span>
                <button @click="nextMonth" class="bg-blue-500 text-white px-2 py-1 rounded">❯</button>
            </div>
            <div class="grid grid-cols-7 gap-1 text-center font-bold">
                <div v-for="day in daysOfWeek" :key="day">{{ day }}</div>
            </div>
            <div class="grid grid-cols-7 gap-1 text-center">
                <div v-for="blank in blanks" :key="'blank' + blank" class="p-2"></div>
                <div v-for="(day, index) in daysInMonth" :key="index" @click="selectDate(day)"
                    class="p-2 cursor-pointer hover:bg-blue-200">
                    {{ day }}
                </div>
            </div>
        </div>
        <div v-if="selectedDate" class="mt-4">
            <h3 class="text-xl font-semibold mb-2">Add Event on {{ selectedDate }}</h3>
            <form @submit.prevent="addEvent" class="space-y-2">
                <input v-model="newEvent.title" placeholder="Title" required class="w-full p-2 border rounded" />
                <input v-model="newEvent.description" placeholder="Description" required
                    class="w-full p-2 border rounded" />
                <input v-model="newEvent.time" type="time" class="w-full p-2 border rounded" />
                <button type="submit" class="bg-green-500 text-white px-4 py-2 rounded">Add Event</button>
            </form>
        </div>
        <h3 class="text-xl font-semibold mt-6">Upcoming Events</h3>
        <ul class="list-disc pl-5 mt-2">
            <li v-for="(event, index) in futureEvents" :key="index">
                <strong>{{ event.date }} {{ event.time }}</strong>: {{ event.title }} - {{ event.description }}
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
        createEvent
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

    const currentMonthName = computed(() => {
        return new Date(currentYear.value, currentMonth.value).toLocaleString('default', {
            month: 'long'
        });
    });

    const futureEvents = computed(() => {
        const today = new Date().toISOString().split('T')[0];
        return events.value.filter(event => event.date >= today);
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
    }

    async function addEvent() {
        const result = await createEvent(newEvent.value.title, newEvent.value.description, selectedDate.value,
            newEvent.value.time || null);
        console.log('Event creation result:', result); // Debug
        newEvent.value = {
            title: '',
            description: '',
            time: ''
        };
        selectedDate.value = null;
        fetchEvents();
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