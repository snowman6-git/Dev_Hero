<script lang="ts">
  import "../app.css"
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from '@tauri-apps/api/event';

  import TerminalTextBlock from "$lib/components/TerminalTextBlock.svelte";
  
  // 나중에 비주얼 노벨 엔진에서 쓸 sqlite에서 멘트 받아오기 text|duration
  // 차후 백앤드 기준, 테이블 1로드 -> 종료시 멘트추가식으로 실제 로딩 화면으로 구현
  let loading_Assets = [ // 재사용가능하게 변수명 그냥 로드 에셋으로 변경하고 다른 로딩때도 쓰자 
    { text: "Initializing...", duration: 2 },
    { text: "load assets", duration: 0.5 },
    { text: "load text assets", duration: 0.25 },
    { text: "load audio assets", duration: 0.25 },
    { text: "load image assets", duration: 0.25 },
    { text: "Complete!", duration: 0.5 },
  ]
  let Terminal_display = $state<any[]>([]);


  $effect(() => {
    // Rust에서 보낸 "loading-step" 이벤트를 구독
    const unlisten = listen<string>("loading-step", (event) => {
      Terminal_display.push(event.payload); // 데이터 오면 즉시 화면에 반영
    });

    return () => { unlisten.then(f => f()); }; // 정리
  });

  function start() {
    invoke("start_real_loading"); // Rust에게 시작하라고 신호 보냄
  }
  start()

  // const opening = async() => {
  //   for (const asset of loading_Assets) {
  //     Terminal_display.push(asset);
  //     await new Promise(resolve => setTimeout(resolve, asset.duration * 1000));
  //   }
  // }
  // opening();




  // async function greet(event: Event) {
    // event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  // }

</script>

<main>
  <div id="terminal">
    <!-- 러스트 + SQL에서 실제 로딩중인 에셋을 문구로 받아오기 EX(테이블 요청 123을 나누고, 가져오기 성공한순으로) -->
		{#each Terminal_display as block}
			<TerminalTextBlock
				text={block.text} 
			/>
		{/each}

  </div>
</main>


<style>

#terminal{
  padding: 1rem;
  box-sizing: border-box;
  /* border: 0.25rem solid white; */
  width: 100vw;
  height: 100vh;
}

</style>
