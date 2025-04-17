<script lang="ts" module>
  export const logStore = logStoreFactory()

  function logStoreFactory() {
    let logs: String[] = $state([])

    function addToStore(updates: String[]) {
      logs = [...logs, ...updates]
    }

    function readLogs() {
      return logs
    }

    return { addToStore, readLogs }
  }
</script>
