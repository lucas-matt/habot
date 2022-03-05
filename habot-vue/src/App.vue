<template>
  <div class="content-container is-fullheight has-background-light-blue">
    <router-view />
  </div>
</template>

<script>
import store from '@/store';

export default {
  name: 'App',
  store,

  components: {
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

body {
  height: 100vh;
}

html {
  height: 100vh;
  width: 100%;
}

.content-container {
  height: 100vh;
}

.has-background-light-blue {
  background: rgb(255,255,255);
  background: linear-gradient(0deg, rgba(255,255,255,1) 5%, rgba(206,247,255,1) 100%);
}

</style>
