// ==UserScript==
// @name     InsertScript
// @version  1
// @include  https://akigeo.com/wieviele/*
// @include  akigeo.com/wieviele/*
// @include  http://127.0.0.1:8000/*
// @include  127.0.0.1:8000/*
// @grant 	 GM.xmlHttpRequest
// ==/UserScript==

const DB_LINK = "https://servers.neumann.ovh:7000/database";
const USERNAME = "online_sammeln_bot";
const PASSWORD = "XyN8sY@wyAH!@hZq"
const ITERATION_DELAY = 750;
const RESPONSE_WAIT_DELAYY = 300;
const REPEAT_AMOUNT = 15;

var sessionToken = "test";
var gemeindeNamenList = [];
var latestTimestamp = null;
var inputElement = document.getElementById("right_top_2_input_id");


console.log("Script started");

async function loginAndGetSessionToken(){
  console.log("Started login attempt")
  let waiting = true;
  let requestPromise = new Promise((resolve, reject) => {
    async function responseHandler(response) {
      console.log(response);
      if (response["status"] != 202)
        console.log(response);
      //return await loginAndGetSessionToken();
      token = response["responseText"];
      resolve(token)
    };

    GM.xmlHttpRequest({
      url: DB_LINK + "/login",
      headers: {
        "Content-Type": "application/json"
      },
      method: "POST",
      data: JSON.stringify({
        "username": USERNAME,
        "password": PASSWORD
      }),
      onload: async function(response) {responseHandler(response);}
    });
  });
  sessionToken = await requestPromise;
}

async function requestNewGemeindeNamenFromDatabase() {
  let requestPromise = new Promise((resolve, reject) => {
    async function responseHandler(response) {
      console.log(response);
      if (response["status"] == 403) {
        resolve(await loginAndGetSessionToken());
      }
      if (response["status"] != 202) {
        console.log(response);
      }
      result = JSON.parse(response["responseText"]);
      console.log(result);
      resolve([result.gemeinde_namen, result.last_timestamp]);
    }
    GM.xmlHttpRequest({
      url: DB_LINK + "/gemeinden",
      headers: {
        "Content-Type": "application/json"
      },
      method: "POST",
      data: JSON.stringify({
        "username": USERNAME,
        "token": sessionToken,
        "starting_timestamp": latestTimestamp
      }),
      onload: async function(response){responseHandler(response);}
    });
  });
  requestResult = await requestPromise;
  gemeindeNamenList = requestResult[0];
  latestTimestamp = requestResult[1] != null ? requestResult[1] : latestTimestamp;
  console.log(latestTimestamp);
}


async function insertNameIntoField(name) {

  for (let i = 0; i < REPEAT_AMOUNT; i++){
    inputElement.value = name;
    inputElement.dispatchEvent(new KeyboardEvent('keyup', {'key': 'Enter'}));
  }

  /*const result = unsafeWindow.solo.search_place(name, unsafeWindow.dataset.names, unsafeWindow.found);
  if (result === undefined) {
    return;
  } else if (Object.prototype.hasOwnProperty.call(found, result)) {
    inputElement.value = "";
    return;
  } else {
    insertNameIntoField(name);
  }*/
}

// This function will be called every ITERATION_DELAY ms due to iterationLoop later on
async function mainIteration(){
  if (gemeindeNamenList.length == 0) {
    await requestNewGemeindeNamenFromDatabase();
  }
  else {
    await insertNameIntoField(gemeindeNamenList.pop());
  }
}

async function iterationLoop(){
  var startTime = Date.now();
  var result = await mainIteration();
  var stopTime = Date.now();
  var elapsed = stopTime - startTime;
  var remainingTimeout = Math.max(ITERATION_DELAY - elapsed, 0);
  setTimeout(iterationLoop, remainingTimeout);
}

// The main body of our Program
(async function(){
  'use strict';

  await loginAndGetSessionToken();
  console.log("Login Attempt completed, moving on to Iteration Loop")
  await iterationLoop();
})();