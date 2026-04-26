<script lang="ts">
  import "../app.css"
  import { invoke } from "@tauri-apps/api/core";
  
  let terminal_text = ""
  
  // 나중에 비주얼 노벨 엔진에서 쓸 sqlite에서 멘트 받아오기
  let boot_up_text: Array<String> = [
    "Initializing...",
    ""
  ]



  // async function greet(event: Event) {
    // event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  // }

</script>

<main>
  <div id="terminal">{terminal_text}</div>
</main>


<style>

#terminal{
  box-sizing: border-box;
  /* border: 0.25rem solid white; */
  width: 100vw;
  height: 100vh;
}

</style>
