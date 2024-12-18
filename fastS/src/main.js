const { invoke } = window.__TAURI__.core;

import { isPathIsFile, toastOn, splitFileName } from "./common.js";

let inputSearchMain = document.querySelector("#id-input-search-main");
let inputSearchExt = document.querySelector("#id-input-search-ext");
let inputCheckHidden = document.querySelector("#id-check-hidden");
let inputCheckStrict = document.querySelector("#id-check-strict");
let inputLocation = document.querySelector("#id-input-location");

let resultOut = document.querySelector("#id-select-out");

let timerId_check;

var bSearchDone = false;
var bDisplayDone = false;

//-------------------------------------------------------------------
//initialize
//-------------------------------------------------------------------
function init() {
  //default setup
  inputSearchExt.disabled = false;
  inputSearchMain.textContent = "";
  inputSearchExt.textContent = "";	
  inputCheckHidden.checked = true;
	inputCheckStrict.checked = true;
  inputLocation.textContent = "";
  
  bDisplayDone = false;
}

init();

async function run() {
  bSearchDone = false;
  bDisplayDone = false;
  
  var callbackName = "do_search_file";
  let tempExt;
  let tempMain;

  if (inputSearchExt.disabled == true) {
    var temp = splitFileName(inputSearchMain.value);
    tempMain = temp[0];
    tempExt = temp[1];
  } else {
    tempMain = inputSearchMain.value;
    tempExt = inputSearchExt.value;
    if (tempExt.length == 0) { //this is a folder
      callbackName = "do_search_dir";
      changeHeaderText("searching directory...");
    }
    else {
      changeHeaderText("searching files...");
    }
  }

  timerId_check = setInterval(() => checkDone(), 1000);

  await invoke(callbackName, { 
    locStart: inputLocation.value, inSearch: tempMain,    
    limit: Number("10"), ext: tempExt,
    depth: Number("3"), hidden: inputCheckHidden.checked, strict: inputCheckStrict.checked
  });
}

async function checkDone() {
	if (bSearchDone == false) {
		if (true == await invoke("rust_check_search_done", {})) {
			clearInterval(timerId_check);
			bSearchDone = true;
			displayList();
			changeHeaderText("What are you looking for?");
		}
	}	
}

async function displayList() {
  if (bDisplayDone == false) {
    resultOut.options.length = 0;

    let v = await invoke("rust_make_vec_result", {});
    console.log(v.length);
    for (let i = 0; i < v.length; i++) {
    console.log(v[i]);
    
    var option = document.createElement("option");
    option.innerText = v[i];
    resultOut.appendChild(option);
    }
    bDisplayDone = true;
  }
}

async function setDialog(id) {
  let v = await invoke("rust_open_dir_dialog", {});
  if (id == 0) {
    inputLocation.value = v;
    console.log(v);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  var focusableItems;
  var focusableItems1 = document.querySelectorAll('.focusableInput');
  var focusableItems2 = document.querySelectorAll('.focusable');
  let currentFocusIndex = 0;

  // Set initial focus
  focusableItems = [...focusableItems1, ...focusableItems2];
  focusableItems[currentFocusIndex].focus();

  var btnRun = document.getElementById('id-button-run');
  var btnLocation = document.getElementById('id-button-location');

  function btn_listener(event){
    switch(event.target.id){
      case 'id-button-location':       setDialog(0);       break;
      case 'id-button-run':			       run();				       break;          
    }
  }

  btnRun.addEventListener('click', btn_listener);
  btnLocation.addEventListener('click', btn_listener);
});

function changeHeaderText(newText) {	
	if (headerText !== null) {
		document.getElementById("headerText").innerText = newText;
	}
}