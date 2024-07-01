import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory } from '../../../declarations/organiczer_backend';

const agent = new HttpAgent();
const organiczer_backend = Actor.createActor(idlFactory, { agent, canisterId: "asrmz-lmaaa-aaaaa-qaaeq-cai" });

export async function getAllEvents() {
    try {
        const events = await organiczer_backend.get_all_events();
        console.log('Fetched events:', events); // Debug
        return events;
    } catch (error) {
        console.error('Failed to fetch events:', error); // Debug
        return [];
    }
}

export async function createEvent(title, description, date, time) {
    try {
        console.log('Creating event with:', { title, description, date, time }); // Debug
        const result = await organiczer_backend.create_event(title, description, date, time ? [time] : []);
        console.log('Event created:', result); // Debug
        return result;
    } catch (error) {
        console.error('Failed to create event:', error); // Debug
        return false;
    }
}
