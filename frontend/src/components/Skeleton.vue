<template>
  <div
    :class="[
      'animate-pulse bg-bg-elevated rounded',
      shapeClasses,
      customClass
    ]"
    :style="customStyle"
  ></div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  shape: {
    type: String,
    default: 'rectangle',
    validator: (value) => ['text', 'circle', 'rectangle', 'card', 'avatar', 'button'].includes(value)
  },
  width: {
    type: String,
    default: null
  },
  height: {
    type: String,
    default: null
  },
  class: {
    type: String,
    default: ''
  }
});

const customClass = computed(() => props.class);

const shapeClasses = computed(() => {
  switch (props.shape) {
    case 'text':
      return 'h-4 rounded-md';
    case 'circle':
    case 'avatar':
      return 'rounded-full';
    case 'card':
      return 'rounded-xl';
    case 'button':
      return 'rounded-lg';
    case 'rectangle':
    default:
      return 'rounded-lg';
  }
});

const customStyle = computed(() => {
  const style = {};

  if (props.width) {
    style.width = props.width;
  } else {
    switch (props.shape) {
      case 'text':
        style.width = '100%';
        break;
      case 'circle':
      case 'avatar':
        style.width = '48px';
        break;
      case 'card':
        style.width = '100%';
        break;
      case 'button':
        style.width = '120px';
        break;
      default:
        style.width = '100%';
    }
  }

  if (props.height) {
    style.height = props.height;
  } else {
    switch (props.shape) {
      case 'text':
        style.height = '16px';
        break;
      case 'circle':
      case 'avatar':
        style.height = '48px';
        break;
      case 'card':
        style.height = '120px';
        break;
      case 'button':
        style.height = '40px';
        break;
      default:
        style.height = '40px';
    }
  }

  return style;
});
</script>
