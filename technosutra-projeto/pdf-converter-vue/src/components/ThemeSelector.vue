<template>
  <div class="theme-selector">
    <label class="theme-label">TEMA</label>
    <div class="theme-dropdown">
      <select :value="modelValue" @change="$emit('update:modelValue', $event.target.value)">
        <option v-for="name in themeNames" :key="name" :value="name">{{ name }}</option>
      </select>
      <span class="dropdown-arrow">▼</span>
    </div>
    <span class="theme-desc">{{ currentDesc }}</span>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { THEMES, getThemeNames } from '../themes/index.js'

const props = defineProps({ modelValue: String })
defineEmits(['update:modelValue'])

const themeNames = getThemeNames()
const currentDesc = computed(() => THEMES[props.modelValue]?.desc || '')
</script>

<style scoped>
.theme-selector { display: flex; align-items: center; gap: 12px; }
.theme-label { 
  font-size: 10px; font-weight: 700; color: #22c55e; letter-spacing: 2px;
  text-shadow: 0 0 10px rgba(34,197,94,0.3);
}
.theme-dropdown { position: relative; }
.theme-dropdown select {
  appearance: none;
  background: linear-gradient(180deg, #1a1a1a 0%, #141414 100%);
  border: 1px solid #262626;
  border-radius: 8px;
  padding: 8px 36px 8px 14px;
  color: #e5e5e5;
  font-weight: 600; font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
  min-width: 160px;
}
.theme-dropdown select:hover { border-color: #22c55e; box-shadow: 0 0 15px rgba(34,197,94,0.1); }
.theme-dropdown select:focus { outline: none; border-color: #22c55e; }
.dropdown-arrow { 
  position: absolute; right: 12px; top: 50%; transform: translateY(-50%);
  font-size: 8px; color: #525252; pointer-events: none;
}
.theme-desc { 
  font-size: 10px; color: #525252; font-style: italic;
  max-width: 200px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
}
</style>
