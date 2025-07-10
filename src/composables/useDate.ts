import { DateTime } from 'luxon';

export const useDate = () => {
    const now = DateTime.now();

    const dateShortcuts = {
        'Aujourd\'hui': () => {
            const today = DateTime.now().startOf('day');
            return [today.toMillis(), today.endOf('day').toMillis()] as [number, number];
        },
        'Cette semaine': () => {
            const startOfWeek = DateTime.now().startOf('week');
            const endOfWeek = DateTime.now().endOf('week');
            return [startOfWeek.toMillis(), endOfWeek.toMillis()] as [number, number];
        },
        'Ce mois': () => {
            const startOfMonth = DateTime.now().startOf('month');
            const endOfMonth = DateTime.now().endOf('month');
            return [startOfMonth.toMillis(), endOfMonth.toMillis()] as [number, number];
        },
        'Cette année': () => {
            const startOfYear = DateTime.now().startOf('year');
            const endOfYear = DateTime.now().endOf('year');
            return [startOfYear.toMillis(), endOfYear.toMillis()] as [number, number];
        },
        'Tous': () => {
            return [0, DateTime.now().endOf('day').toMillis()] as [number, number];
        }
    };

    const isSelectedDate = (date: [number, number], selectedDate: [number, number]): boolean => {
        return date[0] === selectedDate[0] && date[1] === selectedDate[1];
    };

    return {
        dateShortcuts,
        listShortcuts: Object.keys(dateShortcuts),
        isSelectedDate,
        now: now.toMillis()
    };
};