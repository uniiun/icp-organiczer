<template>
    <div class="p-4 bg-white rounded shadow-md">
        <h2 class="text-2xl font-bold mb-4 text-center">Event Calendar</h2>
        <div>
            <h3 class="text-xl font-semibold mt-6">Add Event</h3>
            <form @submit.prevent="addNewEvent" class="space-y-2">
                <input v-model="newEvent.title" placeholder="Title" required class="w-full p-2 border rounded" />
                <input v-model="newEvent.description" placeholder="Description" class="w-full p-2 border rounded" />
                <input v-model="newEvent.date" type="date" class="w-full p-2 border rounded" required />
                <input v-model="newEvent.time" type="time" class="w-full p-2 border rounded" />
                <button type="submit" class="bg-green-500 text-white px-4 py-2 rounded">Add Event</button>
            </form>
        </div>
        <div v-if="events.length" class="mt-4">
            <h3 class="text-xl font-semibold">All Events</h3>
            <ul class="list-disc pl-5 mt-2">
                <li v-for="event in events" :key="event.id">
                    <strong>{{ event.date }} {{ event.time }}</strong>: {{ event.title }} - {{ event.description || 'No description' }}
                    <button @click="startEditEvent(event.id)" class="bg-yellow-500 text-white px-2 py-1 rounded ml-2">Edit</button>
                    <button @click="removeEvent(event.id)" class="bg-red-500 text-white px-2 py-1 rounded ml-2">Delete</button>
                    <div v-if="event.id === editEventId" class="mt-2">
                        <form @submit.prevent="saveUpdatedEvent(event.id)" class="space-y-2">
                            <input v-model="editEventData.title" placeholder="Title" required class="w-full p-2 border rounded" />
                            <input v-model="editEventData.description" placeholder="Description" class="w-full p-2 border rounded" />
                            <input v-model="editEventData.date" type="date" class="w-full p-2 border rounded" required />
                            <input v-model="editEventData.time" type="time" class="w-full p-2 border rounded" />
                            <button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded">Update Event</button>
                        </form>
                    </div>
                </li>
            </ul>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { getAllEvents, createEvent, updateEvent as updateExistingEvent, deleteEvent as deleteExistingEvent } from '../api/Calendar.js';

const events = ref([]);
const newEvent = ref({ title: '', description: '', date: '', time: '' });
const editEventId = ref(null);
const editEventData = ref({ title: '', description: '', date: '', time: '' });

async function fetchEvents() {
    events.value = await getAllEvents();
}

async function addNewEvent() {
    await createEvent(newEvent.value.title, newEvent.value.description || null, newEvent.value.date, newEvent.value.time || null);
    newEvent.value = { title: '', description: '', date: '', time: '' };
    fetchEvents();
}

function startEditEvent(id) {
    const event = events.value.find(event => event.id === id);
    editEventId.value = id;
    editEventData.value = { ...event };
}

async function saveUpdatedEvent(id) {
    await updateExistingEvent(id, editEventData.value.title, editEventData.value.description || null, editEventData.value.date, editEventData.value.time || null);
    editEventId.value = null;
    fetchEvents();
}

async function removeEvent(id) {
    await deleteExistingEvent(id);
    fetchEvents();
}

onMounted(fetchEvents);
</script>

<style scoped>
</style>
