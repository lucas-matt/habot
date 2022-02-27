import { createStore } from 'vuex';

const store = createStore({
    state() {
        return {
            msalConfig: {
                auth: {
                    clientId: '8064f24d-260c-4abb-87f1-21e0ccf342d5',
                    authority:
                        'https://login.microsoftonline.com/consumers',
                },
                cache: {
                    cacheLocation: 'localStorage',
                }
            },
            accessToken:""
        };
    },
    mutations :{
        setAccessToken(state, token){
            state.accessToken = token;
        }
    }
});

export default store;