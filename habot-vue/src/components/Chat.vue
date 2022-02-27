<template>
  <div class="container is-fullheight chat-box">
    <div id="webchat" class="container" role="main"></div>
  </div>
</template>

<script>

import {PublicClientApplication} from "@azure/msal-browser";

function initials(account) {
  let name = account.name;
  let names = name.split(" ");
  if (names.length >= 2) {
    return `${names[0].charAt(0)}${names[names.length-1].charAt(0)}`
  }
  return "UK"
}

export default {
  name: 'Chat',
  components: { },
  props: [],
  data () {
    return {
      account: undefined
    }
  },
  computed: {

  },
  async created() {
    this.$msalInstance = new PublicClientApplication(
        this.$store.state.msalConfig,
    );
  },
  mounted () {

    const accounts = this.$msalInstance.getAllAccounts();
    if (accounts.length === 0) {
      return;
    }
    this.account = accounts[0];

    // hack to load webchat script
    const webchatScript = document.createElement('script');
    webchatScript.type = 'text/javascript';
    webchatScript.src = '/webchat.js';
    document.body.appendChild(webchatScript);

    // wait for script to load before creating webchat
    webchatScript.addEventListener('load', () => {

      // Set style options.
      let userAvatarInitials = initials(this.account);
      console.log(userAvatarInitials);
      const styleOptions = {
        botAvatarInitials: 'BF',
        userAvatarInitials: userAvatarInitials
      };

      window.WebChat.renderWebChat(
          {
            directLine: window.WebChat.createDirectLine({
              token: '4Z2hRMp54Yc.Ml__NKAb2nhLldy2bePqOaspkwhw__bh_SGtlk1WvY8'
            }),
            userID: 'YOUR_USER_ID',
            username: 'Web Chat User',
            styleOptions
          },
          document.getElementById('webchat')
      );
    })

  },
  methods: {

  }
}
</script>

<style lang="scss">

@import '@/styles/index.scss';

#webchat {
  width: 85%;
}

</style>