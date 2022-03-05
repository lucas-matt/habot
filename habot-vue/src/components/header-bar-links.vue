<template>
  <div class="navbar-menu m-1">
<!--    <div class="navbar-start">-->
<!--      <div class="navbar-item">-->
<!--        <router-link v-if="account" :to="'/' + accountId + '/coach'">Coach</router-link>-->
<!--      </div>-->
<!--      <div class="navbar-item">-->
<!--        <router-link v-if="account" :to="'/' + accountId + '/progress'">Progress</router-link>-->
<!--      </div>-->
<!--    </div>-->
    <div class="navbar-end">
      <div class="navbar-item">
        <div class="navbar-item" v-if="account">
          {{account.name}}
        </div>
        <div class="navbar-divider" />
        <div class="buttons">
          <button
              @click="SignOut"
              v-if="account"
              target="_blank"
              class="button is-black m-3"
              rel="noopener noreferrer"
          >
            <img class="logo" src="@/assets/microsoft.png"/>
            Sign-out
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {PublicClientApplication} from '@azure/msal-browser';

export default {
  name: 'HeaderBarLinks',
  data() {
    return {
      account: undefined,
      accountId: undefined,
      name: undefined,
      signin: 'https://microsoft.com',
    };
  },
  async created() {
    this.$msalInstance = new PublicClientApplication(
        this.$store.state.msalConfig,
    );
  },
  mounted() {
    const accounts = this.$msalInstance.getAllAccounts();
    if (accounts.length == 0) {
      this.accountId = "none";
      return;
    }
    this.account = accounts[0];
    this.accountId = this.account.localAccountId;
    this.$emitter.emit('login', this.account);
  },
  methods: {
    async SignIn() {
      await this.$msalInstance
          .loginPopup({})
          .then(() => {
            const myAccounts = this.$msalInstance.getAllAccounts();
            this.account = myAccounts[0];
            this.$emitter.emit('login', this.account);
          })
          .catch(error => {
            console.error(`error during authentication: ${error}`);
          });
    },
    async SignOut() {
      await this.$msalInstance
          .logout({})
          .then(() => {
            this.$emitter.emit('logout', 'logging out');
          })
          .catch(error => {
            console.error(error);
          });
    },
  },
};
</script>

<style>
.logo {
  height: 24px;
  width: 24px;
  margin: 0.75rem;
}
</style>