<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from '@tauri-apps/api/event';
  import TerminalTextBlock from "$lib/components/TerminalTextBlock.svelte";
  import { onMount } from "svelte";

  // 나중에 비주얼 노벨 엔진에서 쓸 sqlite에서 멘트 받아오기 text|duration
  // 차후 백앤드 기준, 테이블 1로드 -> 종료시 멘트추가식으로 실제 로딩 화면으로 구현
  let Terminal_display = $state<any[]>([]);

  $effect(() => {
    // Rust에서 보낸 이벤트를 구독
    const unlisten = listen<string>("init", (event) => {
      let init_log: string = event.payload
      Terminal_display.push({"text": init_log}); // 데이터 오면 즉시 화면에 반영
      // 정말 이걸로 괜찮을지 생각하기
      if (init_log == "system_Ready"){
        location.href = "title"
      }
    });
    return () => { unlisten.then(f => f()); }; // 정리
  });
  onMount(() => {
    invoke("init"); // Rust에게 시작하라고 신호 보냄
  });


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
