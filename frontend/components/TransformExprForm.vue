<script setup lang="ts">
import type { TransformExpr } from "../utils/assets";
import PhVectorThree from "../components/icons/ph/vector-three.vue";
import MdiAxisZRotateCounterclockwise from "../components/icons/mdi/axis-z-rotate-counterclockwise.vue";

defineProps<{
  exprs: TransformExpr;
}>();

const trans_placeholders = ["x", "y", "z"];
const rot_placeholders = ["x", "y", "z", "w"];
</script>

<template>
  <div class="flex space-x-2 ml-2 flex-col">
    <div class="flex items-center gap-2">
      <PhVectorThree />
      <input
        v-for="(_, idx) in exprs.t_exprs"
        :key="'trans-' + trans_placeholders[idx]"
        v-model.string="exprs.t_exprs[idx]"
        type="text"
        :placeholder="trans_placeholders[idx]"
        class="p-1 border rounded w-26"
      />
    </div>
    <div class="flex items-center gap-2">
      <MdiAxisZRotateCounterclockwise />
      <input
        v-for="(_, idx) in exprs.r_exprs"
        :key="'trans-' + trans_placeholders[idx]"
        v-model.string="exprs.r_exprs[idx]"
        type="text"
        :placeholder="rot_placeholders[idx]"
        class="p-1 border rounded w-26"
      />
    </div>
  </div>
</template>
