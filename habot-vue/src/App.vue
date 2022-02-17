<template>
  <HeaderBar />
  <img alt="Vue logo" src="./assets/logo.png">
  <HelloWorld msg="Welcome to Your Vue.js App"/>
</template>

<script>
import HeaderBar from '@/components/header-bar';
import HelloWorld from '@/components/HelloWorld';
import store from '@/store';

export default {
  name: 'App',
  store,
  components: {
    HelloWorld, HeaderBar
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
  margin-top: 60px;
}
</style>

<style lang="scss">
@import '@/styles/index.scss';
</style>
