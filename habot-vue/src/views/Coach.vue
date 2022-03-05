<template>
  <div class="is-fullheight">
    <HeaderBar/>
    <section class="container hero has-shadow">
      <div class="hero-head has-text-white">
      </div>
      <div class="hero-container has-background-white">
        <div class="hero-body is-vcentered">
          <Chat direct-line-secret="yourSecret_3g9h459gb4etrvo8e4bter4tb"/>
        </div>
      </div>
    </section>
    <footer class="foot has-background-grey-lighter">
      <div class="content has-text-centered">
        <p>

        </p>
      </div>
    </footer>
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
  height: 100%;
}

.hero-container {
  height: 75vh;
  border-radius: 0.5em;
  padding: 1em;
}

.foot {
  position: absolute;
  bottom: 10px;
  width: 100%;
}

</style>