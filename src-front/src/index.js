import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/tauri";

listen('tauri://resize',(e)=>{
    console.log('resize')
})

listen("somaemit",(e)=>{
    console.log('its some emit')
})

const btn = document.querySelector('.btn');
btn.addEventListener('click',(e)=>{
    console.log('btn start');
    invoke('my_custom_command');
})