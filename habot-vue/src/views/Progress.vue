<template>
  <div class="content-container">
    <HeaderBar />
    <h1>Progress</h1>
  </div>
</template>

<script>
import HeaderBar from '@/components/header-bar'
import store from "@/store";

export default {
  name: 'Progress',
  store,

  components: {
    HeaderBar
  }
}
</script>