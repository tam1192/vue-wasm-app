<script setup lang="ts">
  import { ref } from 'vue';
  import init, {math_parse} from '../wasm/wasm_rs';

  init().then(() => {
    const result = ref(0);
    const text = ref('')


    function onInput() {
      const ans = math_parse(text.value) ?? 0;
      result.value = ans;
    }
  });
</script>