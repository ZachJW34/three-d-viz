<script setup lang="ts">
import { computed, ref } from "vue";
import {
  type Asset,
  genDefaultTransformStartEnd,
  genDefaultTransformExpr,
  type PrimitiveKey,
  genPrimitive,
  PrimitiveGenerators,
  genChild,
  generateSafeName,
} from "../utils/assets";
import TransformForm from "./TransformForm.vue";
import TransformExprForm from "./TransformExprForm.vue";
import MaterialSymbolsAdd2RoundedIcon from "./icons/material-symbols/add-2-rounded.vue";
import MaterialSymbolsDeleteIcon from "./icons/material-symbols/delete.vue";
import FunctionIcon from "./icons/mdi/function.vue";
import PhVectorThreeIcon from "./icons/ph/vector-three.vue";
import MaterialSymbolsArrowDropDownRoundedIcon from "./icons/material-symbols/arrow-drop-down-rounded.vue";

const props = defineProps<{ asset: Asset; siblings: Asset[] }>();
const emits = defineEmits<{ delete: [asset: Asset] }>();
const isOpen = ref(true);

function changeTransformType(asset: Asset) {
  if (asset.animator.type === "StartEnd") {
    asset.animator = {
      type: "Expr",
      value: genDefaultTransformExpr(),
    };
  } else {
    asset.animator = {
      type: "StartEnd",
      value: genDefaultTransformStartEnd(),
    };
  }
}

function addChild() {
  props.asset.children.push(genChild(props.asset.children));
}

function deleteChild(asset: Asset) {
  props.asset.children = props.asset.children.filter((a) => a.id !== asset.id);
}

const primitiveSelect = computed(() => props.asset.primitive.type);
function onSelectPrimitive(e: Event) {
  const key = (e.target as any).value as PrimitiveKey;
  props.asset.primitive = genPrimitive(key);
  props.asset.name = generateSafeName(
    props.asset.primitive.type,
    props.siblings
  );
}
const primitive = computed(() => {
  const { type, ...rest } = props.asset.primitive;
  return rest as unknown as Record<string, any>;
});
</script>

<template>
  <div class="relative">
    <form
      id="transformation-form"
      :class="[
        'border rounded border-gray-400 p-2 flex flex-col relative text-xs gap-2 mb-2',
        { 'h-11 overflow-hidden': !isOpen, 'h-44': isOpen },
      ]"
    >
      <div class="flex h-6 items-center">
        <h2>{{ props.asset.name }}</h2>
        <button class="cursor-pointer" @click="isOpen = !isOpen" type="button">
          <MaterialSymbolsArrowDropDownRoundedIcon
            width="16"
            height="16"
            :class="{ 'rotate-180': isOpen }"
          />
        </button>
      </div>
      <div class="absolute top-2 right-2 flex gap-1">
        <button
          type="button"
          @click="addChild()"
          class="border border-gray-400 rounded hover:bg-gray-700 p-1 h-6 w-6"
        >
          <MaterialSymbolsAdd2RoundedIcon />
        </button>
        <button
          type="button"
          @click="emits('delete', asset)"
          class="border border-gray-400 rounded hover:bg-gray-700 p-1 h-6 w-6"
        >
          <MaterialSymbolsDeleteIcon />
        </button>
      </div>
      <template v-if="isOpen">
        <div class="flex items-center gap-2">
          <div class="flex flex-col gap-1">
            <h3 class="font-semibold underline w-13" for="primitive">
              Primitive
            </h3>
            <div class="ml-2 flex items-center gap-2">
              <label for="shape" class="w-12">Shape:</label>
              <select
                name="shape"
                :value="primitiveSelect"
                @change="(e) => onSelectPrimitive(e)"
                class="border border-white rounded p-1"
              >
                <option
                  v-for="(_, key) in PrimitiveGenerators"
                  :key="key"
                  :value="key"
                >
                  {{ key }}
                </option>
              </select>
              <template v-for="(_, key) in primitive" :key="key">
                <label for="key">{{ key }}:</label>
                <input
                  class="p-1 border rounded w-16"
                  name="key"
                  type="number"
                  min="0"
                  v-model.number="(props.asset.primitive as unknown as any)[key]"
                />
              </template>
            </div>
          </div>
        </div>
        <div class="flex flex-col gap-1">
          <h3 class="underline font-semibold">Transform</h3>
          <div class="flex">
            <button
              type="button"
              class="border-gray-400 border rounded p-1 ml-2 hover:bg-gray-700 cursor-pointer"
              @click="changeTransformType(asset)"
            >
              <component
                :is="
                  asset.animator.type === 'StartEnd'
                    ? PhVectorThreeIcon
                    : FunctionIcon
                "
              />
            </button>
            <template v-if="asset.animator.type === 'StartEnd'">
              <div class="ml-2">
                <div class="flex items-center">
                  <label class="w-10">Start:</label>
                  <TransformForm :transform="asset.animator.value.start" />
                </div>
                <div class="flex items-center">
                  <label class="w-10">End:</label>
                  <TransformForm :transform="asset.animator.value.end" />
                </div>
              </div>
            </template>
            <template v-else>
              <TransformExprForm :exprs="asset.animator.value" />
            </template>
          </div>
        </div>
      </template>
    </form>
    <template v-if="isOpen">
      <template v-for="asset in asset.children">
        <Asset
          :asset="asset"
          :siblings="props.asset.children"
          @delete="deleteChild"
          class="ml-4"
        />
      </template>
    </template>
  </div>
</template>
