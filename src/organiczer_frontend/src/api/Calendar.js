import {
    Actor,
    HttpAgent
} from '@dfinity/agent';
import {
    idlFactory
} from '../../../declarations/organiczer_backend';

const agent = new HttpAgent();

if (process.env.DFX_NETWORK === 'local') {
    agent.fetchRootKey().catch(err => {
        console.warn('Unable to fetch root key. Check to ensure that your local replica is running');
        console.error(err);
    });
}

const organiczer_backend = Actor.createActor(idlFactory, {
    agent,
    canisterId: "asrmz-lmaaa-aaaaa-qaaeq-cai"
});

export async function getAllEvents() {
    try {
        const events = await organiczer_backend.get_all_events();
        console.log('Fetched events:', events);
        return events;
    } catch (error) {
        console.error('Failed to fetch events:', error);
        return [];
    }
}

export async function createEvent(title, description, date, time) {
    try {
        console.log('Creating event with:', {
            title,
            description,
            date,
            time
        });
        const result = await organiczer_backend.create_event(title, description ? [description] : [], date, time ? [time] : []);
        console.log('Event created:', result);
        return result;
    } catch (error) {
        console.error('Failed to create event:', error);
        return false;
    }
}

export async function updateEvent(id, title, description, date, time) {
    try {
        console.log('Updating event with:', {
            id,
            title,
            description,
            date,
            time
        });
        const result = await organiczer_backend.update_event(id, title, description ? [description] : [], date, time ? [time] : []);
        console.log('Event updated:', result);
        return result;
    } catch (error) {
        console.error('Failed to update event:', error);
        return false;
    }
}

export async function deleteEvent(id) {
    try {
        console.log('Deleting event with id:', id);
        const result = await organiczer_backend.delete_event(id);
        console.log('Event deleted:', result);
        return result;
    } catch (error) {
        console.error('Failed to delete event:', error);
        return false;
    }
}