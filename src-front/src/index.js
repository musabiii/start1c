import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/tauri";

listen('tauri://resize',(e)=>{
    console.log('resize')
})