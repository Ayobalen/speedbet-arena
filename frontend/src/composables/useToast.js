// Toast composable for global toast notifications
import { ref, readonly } from 'vue';

const toasts = ref([]);
let toastId = 0;

export function useToast() {
  function showToast({ type = 'info', title, message, duration = 5000 }) {
    const id = ++toastId;
    toasts.value.push({ id, type, title, message, duration });

    // Auto-remove after duration
    if (duration > 0) {
      setTimeout(() => {
        removeToast(id);
      }, duration + 300); // Extra time for exit animation
    }

    return id;
  }

  function removeToast(id) {
    const index = toasts.value.findIndex(t => t.id === id);
    if (index > -1) {
      toasts.value.splice(index, 1);
    }
  }

  function success(title, message) {
    return showToast({ type: 'success', title, message });
  }

  function error(title, message) {
    return showToast({ type: 'error', title, message });
  }

  function warning(title, message) {
    return showToast({ type: 'warning', title, message });
  }

  function info(title, message) {
    return showToast({ type: 'info', title, message });
  }

  return {
    toasts: readonly(toasts),
    showToast,
    removeToast,
    success,
    error,
    warning,
    info
  };
}
