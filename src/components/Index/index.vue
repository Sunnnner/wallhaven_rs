<template>
  <Default :info="parentMsg"></Default>
</template>

<script lang="ts" setup>


import Default from '../Default/index.vue';
import {reactive} from 'vue';

const parentMsg = reactive({
  categories: 111,
  purity: 110,
  topRange: '',
  sorting: "date_added",
  order: "desc",
  ai_art_filter: 1,
  page: 1
});

</script>

<style lang="scss">
@import '../Style/theme.scss';
</style>