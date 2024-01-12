/* @refresh reload */
import { render } from 'solid-js/web'

import './index.css'
import App from './App'

import * as wasm from "chat-server"

wasm.ChatApp.new()
  .then(async (app) => {
    console.log("App created")
    //while (true) {
    //  console.log("tick")
    //  if (app.is_disconnected()) {
    //    console.log("Disconnected")
    //    break
    //  }
    //  app.send_packets()
    //  app.update()
    //  let messages = app.get_messages()
    //  app.clear_messages()
    //  for (let message of messages) {
    //    console.log(message)
    //  }
    //  messages_to_send = []
    //  await new Promise(r => setTimeout(r, 50));
    //}
    //console.log("App disconnected")
  })
  .catch((err) => console.error(err))

const root = document.getElementById('root')

//render(() => <App />, root!)

//const url = 'https://172.20.17.234:3080';
//const transport = new WebTransport(url);
///**
// * @type {WritableStreamDefaultWriter<Uint8Array>}
// */
//let writer;
//let reader;
//
//// Optionally, set up functions to respond to
//// the connection closing:
//transport.closed
//  .then(() => {
//    console.log(`The HTTP/3 connection to ${url} closed gracefully.`);
//  })
//  .catch((error) => {
//    console.error(`The HTTP/3 connection to ${url} closed due to ${error}.`);
//  });
//
//transport.ready.then(async () => {
//  console.log(`The HTTP/3 connection to ${url} is ready.`);
//  reader = transport.datagrams.readable.getReader();
//  writer = transport.datagrams.writable.getWriter();
//  while (true) {
//    const { value, done } = await reader.read();
//    if (done) {
//      console.log('No more datagrams.');
//      break;
//    }
//    console.log(`Datagram received: ${value}`);
//  }
//});
//
//setInterval(() => {
//  if (writer) {
//    writer.write(new Uint8Array([1, 2, 3])).then(() => {
//      console.log('Datagram sent.');
//    });
//  }
//}, 1000);
