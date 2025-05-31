<script setup lang="ts">
  import { ref } from 'vue';
  import init, {math_parse} from './wasm/wasm_rs';

  const result = ref(0);
  const text = ref('')
  // wasm関数が入るところ
  let mp = (_: string): number | undefined => 0;

  // ダークモード判定
  const darkmode = window.matchMedia('(prefers-color-scheme: dark)').matches;
  document.documentElement.setAttribute('data-bs-theme', darkmode ? 'dark' : 'light');

  // wasm読み込み
  init().then(()=>{
    mp = math_parse;
    console.log('wasm loaded');
  });

  // 入力された時に変更するやつ
  function onInput(e:any) {
    text.value=e.target.value;
    const ans = mp(text.value) ?? 0;
    result.value = ans;
  }
</script>

<template>
  <br>
  <h1 class="text-center">計算パーサー！</h1>
  <br>
  <p class="text-center">計算式から結果を出力します。</p>
  <br>
  <br>
  <div class="container text-center">
    <input type="text" placeholder="計算式を入力してください" :value="text" @input="onInput"></input>
    <a> = </a>
    <a>{{ result }}</a>
  </div>
  <br>
  <br>
  <div class="container text-center">
  <h2 class="text-center">これなに？</h2>
  <p>計算式を入力すると、文字列を解析して計算結果を出力します。</p>
  <p>rustの<a href="https://docs.rs/nom/latest/nom/">nom</a>によってパーサーを作成しています。</p>
  <p>ここで一つ疑問になるはずです。なぜシステムレベルの言語であるrustを、ブラウザで動作させられるのでしょうか？</p>
  <p><a href="https://ja.wikipedia.org/wiki/WebAssembly">webassembly</a>という技術によって、rustのコードがビルドされ、ブラウザ上で実行されます。</p>
  <p>どうでもいいけどこの文章半分AIに頼りました。 Qwen2.5-coder:3bのくせに日本語で書いたです。</p>
  <p>そのうち記事にするので、待っててね。</p>
  <br>
  <p><a href="https://adw39.org/">notepad.md(作者のブログ)</a></p>
  <p><a href="https://github.com/tam1192/vue-wasm-app">リポジトリ</a></p>
  <br>
  </div>
</template>