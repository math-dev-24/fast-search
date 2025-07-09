import { reactive, onMounted, watch } from "vue"
import type { Setting } from "../types"


export const useSetting = () => {
    const setting = reactive<Setting>({
        search_path: "/"
    })

    const setSearchPath = async (path: string) => {
        setting.search_path = path;
        await localStorage.setItem('search_path', path);
    }

    const getSearchPath = async () => {
        const path = await localStorage.getItem('search_path');
        if (path) {
            setting.search_path = path;
        }
    }

    onMounted(async () => {
        await getSearchPath();
    })

    watch(() => setting.search_path, (newVal: string) => {
        localStorage.setItem('search_path', newVal);
    })

    return {
        setting,
        setSearchPath,
        getSearchPath,
    }
}