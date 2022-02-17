import { createStore } from 'vuex';

const store = createStore({
    state() {
        return {
            msalConfig: {
                auth: {
                    clientId: 'cb274820-595f-4f53-bd12-bdbf45bfd69b',
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