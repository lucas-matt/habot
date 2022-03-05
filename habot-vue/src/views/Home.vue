<template>
  <div class="content-container is-fullheight has-background-light-blue">
    <section class="hero is-fullheight">
      <div class="hero-body">
        <div class="container">
          <div class="columns is-centered">
            <div class="column is-5-tablet is-4-desktop is-3-widescreen">
              <form action="" class="box">
                <div class="field">
                  <img src="/images/habot-logo.png" />
                </div>
                <div class="field m-3">
                  <form
                      v-on:submit.prevent
                      class="buttons is-centered">
                    <button
                        v-if="!account"
                        @click="SignIn"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="button is-black m-3 is-centered">
                      <img class="logo" src="@/assets/microsoft.png"/>
                      <b>Sign-in with Microsoft</b>
                    </button>
                  </form>
                </div>
              </form>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
import {PublicClientApplication} from '@azure/msal-browser';
import {mapMutations} from 'vuex';

export default {
  name: 'Home',
  data() {
    return {
      account: undefined,
      containers: [],
      signin: 'https://microsoft.com',
    };
  },
  async created() {
    this.$emitter.on(
        'login',
        async function (account) {
          this.account = account;
        }.bind(this),
    ),
        this.$emitter.on('logout', () => {
          console.log('logging out');
          this.account = undefined;
        }),
        this.$msalInstance = new PublicClientApplication(
            this.$store.state.msalConfig,
        );
  },
  mounted() {
    const accounts = this.$msalInstance.getAllAccounts();
    if (accounts.length === 0) {
      return;
    }
    this.account = accounts[0];
    this.$emitter.emit('login', this.account);
    this.$router.push(`/${this.account.localAccountId}/coach`)
  },
  methods: {
    ...mapMutations(['setAccessToken']),
    async getAccessToken() {
      let request = {
        scopes: ['https://storage.azure.com/user_impersonation'],
      };
      const msalInstance = new PublicClientApplication(
          this.$store.state.msalConfig,
      );
      try {
        let tokenResponse = await msalInstance.acquireTokenSilent(request);
        this.$store.commit('setAccessToken', tokenResponse.accessToken);
      } catch (error) {
        console.error('Silent token acquisition failed. Using interactive mode');
        let tokenResponse = await msalInstance.acquireTokenPopup(request);
        console.log(`Access token acquired via interactive auth ${tokenResponse.accessToken}`)
        this.$store.commit('setAccessToken', tokenResponse.accessToken);
      }
    },
    async SignIn() {
      console.log("---")
      await this.$msalInstance
          .loginPopup({})
          .then(() => {
            const myAccounts = this.$msalInstance.getAllAccounts();
            this.account = myAccounts[0];
            this.$emitter.emit('login', this.account);
            this.$router.push(`/${this.account.localAccountId}/coach`)
          })
          .catch(error => {
            console.error(`error during authentication: ${error}`);
          });
    },
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  display: inline-block;
  margin: 0 10px;
}

a {
  color: #42b983;
}

.logo {
  height: 24px;
  width: 24px;
  margin: 0.75rem;
}

.signin {

}

.centre {
  text-align: center;
}

</style>