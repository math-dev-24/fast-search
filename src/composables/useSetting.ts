import { reactive, onMounted, watch } from "vue"
import type { Setting } from "../types"


export const useSetting = () => {
    const setting = reactive<Setting>({
        search_path: ["/"]
    })

    const setSearchPath = async (path: string) => {
        setting.search_path = [path];
        await localStorage.setItem('search_path', setting.search_path.map(reformatePath).join(","));
    }

    const getSearchPath = async () => {
        const paths = await localStorage.getItem('search_path');
        if (paths) {
            setting.search_path = paths.split(",").map(reformatePath);
        }
    }

    const reformatePath = (path: string) => {
        return path.replace(/\\/g, "/");    
    }

    onMounted(async () => {
        await getSearchPath();
    })

    watch(() => setting.search_path, (newVal: string[]) => {
        localStorage.setItem('search_path', newVal.map(reformatePath).join(","));
    })

    return {
        setting,
        setSearchPath,
        getSearchPath,
        reformatePath,
    }
}