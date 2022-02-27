<template>
  <div class="is-fullheight">
    <HeaderBar/>
    <section class="container hero">
      <div class="hero-head has-text-white">
        <p class="title has-text-white-ter">
          Coach
        </p>
      </div>
      <div class="hero-body has-background-white is-vcentered">
        <Chat direct-line-secret="yourSecret_3g9h459gb4etrvo8e4bter4tb"/>
      </div>

    </section>
  </div>
</template>

<script>
import HeaderBar from '@/components/header-bar'
import Chat from '@/components/Chat';
import store from "@/store";

export default {
  name: 'Coach',
  store,

  components: {
    HeaderBar,
    Chat
  },
  mounted() {

  }
}
</script>

<style lang="scss" scoped>

@import '@/styles/index.scss';

.hero-head {
  padding: 2em;
}

.hero-body {
  overflow-y: auto;
  height: 75vh;
}

</style>