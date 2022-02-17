<template>
  <div class="content-container">
    <div class="section content-title-group">
      <h2 class="title">Welcome to the Azure AD + Storage app with Vue.js</h2>
    </div>
    <div v-if="account">
      <div class="level">
        <div class="level-item title">
          You are logged in as {{ account.name }}
        </div>
      </div>
      <div class="level">
        <div class="level-item">
          <ul>
            <li v-for="container in containers" :key="container.id">
              {{ container.name }}
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div v-else>You need to authenticate to access your SQL data</div>
  </div>
</template>

<script>
import { PublicClientApplication } from '@azure/msal-browser';
import { mapMutations } from 'vuex';

export default {
  name: 'HelloWorld',
  data() {
    return {
      account: undefined,
      containers: [],
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
        });
  },
  methods: {
    ...mapMutations(['setAccessToken']),
    async getAccessToken(){
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
        console.error( 'Silent token acquisition failed. Using interactive mode' );
        let tokenResponse = await msalInstance.acquireTokenPopup(request);
        console.log(`Access token acquired via interactive auth ${tokenResponse.accessToken}`)
        this.$store.commit('setAccessToken',tokenResponse.accessToken);
      }
    }
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
</style>