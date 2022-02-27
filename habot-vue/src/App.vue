<template>
  <div class="content-container is-fullheight has-background-info">
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
}

.content-container {
  height: 100vh;
}

</style>
