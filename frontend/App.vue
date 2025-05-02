<script setup lang="ts">
import { ref, watch } from "vue";
import {
  type Asset as IAsset,
  type State,
  bevyUpdateState,
  genChild,
  getStateLS,
  saveStateLS,
} from "./utils/assets";
import MaterialSymbolsSaveIcon from "./components/icons/material-symbols/save.vue";
import MaterialSymbolsAdd2RoundedIcon from "./components/icons/material-symbols/add-2-rounded.vue";
import SketchFabIcon from "./components/icons/simple-icons/sketchfab.vue";

import Asset from "./components/Asset.vue";
import { generateSpherePoints } from "./utils/prefabs";

const menuOpen = ref(false);
const state = ref<State>(getStateLS());

watch(
  state,
  () => {
    try {
      bevyUpdateState(state.value);
    } catch (e) {
      console.log(`Failed to update Bevy State: ${e}`);
    }
  },
  { deep: true }
);

function saveState() {
  saveStateLS(state.value);
}

function addAsset() {
  const newAsset = genChild(state.value.assets);
  state.value.assets.push(newAsset);
}
function addPrefab() {
  state.value.assets.push(generateSpherePoints(100));
}
function deleteChild(asset: IAsset) {
  state.value.assets = state.value.assets.filter((a) => a.id !== asset.id);
}
</script>

<template>
  <div
    id="main-overlay"
    :class="[
      'absolute border rounded border-white p-1 text-white flex flex-col',
      { open: menuOpen },
    ]"
  >
    <div class="flex justify-between gap-4">
      <button
        @click="menuOpen = !menuOpen"
        id="menu-btn"
        :class="[
          'hamburger flex flex-col justify-between items-center w-8 h-8 cursor-pointer z-50 min-w-8',
          { open: menuOpen },
        ]"
      >
        <div class="line1 line w-full h-1 bg-white rounded"></div>
        <div class="line2 line w-full h-1 bg-white rounded"></div>
        <div class="line3 line w-full h-1 bg-white rounded"></div>
      </button>
      <div class="flex gap-2">
        <button
          class="border rounded border-gray-400 p-2 cursor-pointer hover:bg-gray-700"
          @click="addAsset"
        >
          <MaterialSymbolsAdd2RoundedIcon />
        </button>
        <button
          class="border rounded border-gray-400 p-2 cursor-pointer hover:bg-gray-700"
          @click="addPrefab"
        >
          <SketchFabIcon />
        </button>
        <button
          class="border rounded border-gray-400 p-2 cursor-pointer hover:bg-gray-700"
          @click="saveState"
        >
          <MaterialSymbolsSaveIcon />
        </button>
      </div>
    </div>

    <div class="overflow-y-scroll flex flex-col m-2">
      <template v-for="asset in state.assets">
        <Asset :asset="asset" :siblings="state.assets" @delete="deleteChild" />
      </template>
    </div>
  </div>
</template>
