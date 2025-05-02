import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import { bevyUpdateState, getStateLS, initState } from "./utils/assets";

initState();
createApp(App).mount("#app");

addEventListener("TrunkApplicationStarted", (event: any) => {
  console.log(
    "application started - bindings:",
    window.wasmBindings,
    "WASM:",
    event.detail.wasm
  );

  bevyUpdateState(getStateLS());
});
