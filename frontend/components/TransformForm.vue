<script setup lang="ts">
import type { Transform } from "../utils/assets";
import PhVectorThree from "../components/icons/ph/vector-three.vue";
import MdiAxisZRotateCounterclockwise from "../components/icons/mdi/axis-z-rotate-counterclockwise.vue";

defineProps<{
  transform: Transform;
}>();

const trans_placeholders = ["x", "y", "z"];
const rot_placeholders = ["x", "y", "z", "w"];
</script>

<template>
  <div class="flex space-x-2 items-center">
    <PhVectorThree />
    <input
      v-for="(_, idx) in transform.trans"
      :key="'trans-' + trans_placeholders[idx]"
      v-model.number="transform.trans[idx]"
      type="number"
      step="any"
      :placeholder="trans_placeholders[idx]"
      class="p-1 border rounded w-12"
    />
    <MdiAxisZRotateCounterclockwise />
    <input
      v-for="(_, idx) in transform.rot"
      :key="'rot-' + rot_placeholders[idx]"
      v-model.number="transform.rot[idx]"
      type="number"
      step="any"
      :placeholder="rot_placeholders[idx]"
      class="p-1 border rounded w-12"
    />
  </div>
</template>
