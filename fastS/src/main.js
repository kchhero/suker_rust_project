const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
// }

async function run() {
  console.log("run ok");
  var callbackName = "do_search_file";

	await invoke(callbackName, { 
		name: "test"
	});
}

window.addEventListener("DOMContentLoaded", () => {
  var btnRun = document.getElementById('id-button-run');

  function btn_listener(event){
		switch(event.target.id){	
			case 'id-button-run':			 run();				    break;          
		}
	}

  btnRun.addEventListener('click', btn_listener);
});
