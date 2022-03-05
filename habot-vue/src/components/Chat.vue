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

      let bots = [
        '/images/johnny_one.jpg',
        '/images/johnny_two.jpg',
        '/images/johnny_three.jpg',
        '/images/johnny_four.jpg',
      ]

      // Set style options.
      let userAvatarInitials = initials(this.account);
      let firstName = this.account.name.split(" ")[0];

      // https://github.com/microsoft/BotFramework-WebChat/blob/main/packages/api/src/defaultStyleOptions.ts
      // https://habotiv.azurewebsites.net/api/messages
      const styleOptions = {
        botAvatarInitials: 'HB',
        botAvatarImage: bots[Math.floor(Math.random()*bots.length)],
        avatarSize:60,
        transitionDuration: "1s",
        userAvatarInitials: userAvatarInitials,
        bubbleBorderRadius: 5,
        bubbleFromUserBorderRadius: 5,
        bubbleBackground: '#F8F8F8'
      };

      const store = window.WebChat.createStore({}, ({ dispatch }) => next => action => {
        if (action.type === 'DIRECT_LINE/CONNECT_FULFILLED') {
          dispatch({
            type: 'WEB_CHAT/SEND_EVENT',
            payload: {
              name: 'loginevent',
              value: { name: firstName, id: this.account.localAccountId}
            }
          });
        }

        return next(action);
      });

      window.WebChat.renderWebChat(
          {
            directLine: window.WebChat.createDirectLine({
              token: '4Z2hRMp54Yc.Ml__NKAb2nhLldy2bePqOaspkwhw__bh_SGtlk1WvY8',
            }),
            store,
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

#webchat {
  text-align: left;
}

</style>