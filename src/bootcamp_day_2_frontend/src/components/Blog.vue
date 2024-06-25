<template>
      <div>
            <h1>
                  <button @click="pobierzWpisy">oddaj wpisy</button>
                  siema
                  {{ wpisy }}
            </h1>
      </div>
</template>

<script>
import { bootcamp_day_2_backend } from 'declarations/bootcamp_day_2_backend/index';

export default {
      data() {
            return{
                 wpisy: [] 
            }
      },
      methods: {
            async pobierzWpisy(){
                 this.wpisy = await bootcamp_day_2_backend.oddaj_wpisy()
            }
      }
}
</script>