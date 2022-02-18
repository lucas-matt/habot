<template>
  <div class="content-container is-fullheight">
    <HeaderBar />
    <Home msg="Welcome to Your Vue.js App"/>
  </div>
</template>

<script>
import HeaderBar from '@/components/header-bar';
import Home from '@/components/Home';
import store from '@/store';

export default {
  name: 'App',
  store,
  components: {
    Home, HeaderBar
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
</style>

<style lang="scss">
@import '@/styles/index.scss';
</style>
