import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/tauri";
import {WindowManager} from "@tauri-apps/api/window";
import '/style.css';

const twindow = new WindowManager();
// twindow.setDecorations(false);

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

const btnCmd = document.querySelector('.btn-cmd');
btnCmd.addEventListener('click',(e)=>{
    console.log('btn start');
    invoke('btn_cmd');
})


const close = document.querySelector('.close');
close.addEventListener('click',(e)=>{
    invoke('close_window',{name:"musa"});
    // invoke("tauri://")
    twindow.close();
    // twindow.setTitle('some title');
})


const open1c = document.querySelector('.open1c');
open1c.addEventListener('click',(e)=>{
    console.log('btn start');
    invoke('open1c');
})


