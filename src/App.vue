<script setup lang="ts">
  import { ref } from 'vue';
  import init, {math_parse} from './wasm/wasm_rs';

  const result = ref(0);
  const text = ref('')
  let mp = (_: string): number | undefined => 0;

  init().then(()=>{
    const test_ans = math_parse("1 + 2");
    console.log(test_ans);
    mp = math_parse;
    console.log('wasm loaded');
  });

  function onInput(e:any) {
    text.value=e.target.value;
    const ans = mp(text.value) ?? 0;
    result.value = ans;
  }
</script>

<template>
  <h1>計算パーサー！</h1>
  <p>計算式から結果を出力します。</p>
  <input type="text" placeholder="計算式を入力してください" :value="text" @input="onInput"></input>
  <a> = </a>
  <a>{{ result }}</a>
</template>