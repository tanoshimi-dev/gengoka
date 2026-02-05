// Admin Dashboard JavaScript

// Confirm dangerous actions
document.addEventListener('DOMContentLoaded', function() {
    // Add confirmation to delete buttons
    const deleteForms = document.querySelectorAll('form[action*="delete"]');
    deleteForms.forEach(form => {
        if (!form.hasAttribute('onsubmit')) {
            form.addEventListener('submit', function(e) {
                if (!confirm('Are you sure you want to delete this item?')) {
                    e.preventDefault();
                }
            });
        }
    });

    // Add confirmation to suspend buttons
    const suspendForms = document.querySelectorAll('form[action*="suspend"]');
    suspendForms.forEach(form => {
        if (!form.hasAttribute('onsubmit')) {
            form.addEventListener('submit', function(e) {
                if (form.action.includes('unsuspend')) {
                    if (!confirm('Are you sure you want to unsuspend this user?')) {
                        e.preventDefault();
                    }
                }
            });
        }
    });

    // Add confirmation to moderate actions
    const moderateForms = document.querySelectorAll('form[action*="unmoderate"]');
    moderateForms.forEach(form => {
        form.addEventListener('submit', function(e) {
            if (!confirm('Are you sure you want to remove moderation from this content?')) {
                e.preventDefault();
            }
        });
    });

    // Auto-dismiss success messages after 5 seconds
    const successMessages = document.querySelectorAll('.bg-green-100');
    successMessages.forEach(msg => {
        setTimeout(() => {
            msg.style.transition = 'opacity 0.5s ease-out';
            msg.style.opacity = '0';
            setTimeout(() => msg.remove(), 500);
        }, 5000);
    });

    // Enable form validation styling
    const forms = document.querySelectorAll('form');
    forms.forEach(form => {
        form.addEventListener('submit', function(e) {
            const inputs = form.querySelectorAll('input[required], textarea[required], select[required]');
            let valid = true;
            inputs.forEach(input => {
                if (!input.value.trim()) {
                    input.classList.add('border-red-500');
                    valid = false;
                } else {
                    input.classList.remove('border-red-500');
                }
            });
            if (!valid) {
                e.preventDefault();
            }
        });
    });
});

// HTMX event handlers
document.body.addEventListener('htmx:beforeRequest', function(evt) {
    // Show loading indicator
    const target = evt.detail.target;
    if (target) {
        target.style.opacity = '0.5';
    }
});

document.body.addEventListener('htmx:afterRequest', function(evt) {
    // Hide loading indicator
    const target = evt.detail.target;
    if (target) {
        target.style.opacity = '1';
    }
});

// Keyboard shortcuts
document.addEventListener('keydown', function(e) {
    // Escape to close modals/go back
    if (e.key === 'Escape') {
        const cancelBtn = document.querySelector('a[href*="admin/"]');
        if (cancelBtn && cancelBtn.textContent.includes('Cancel')) {
            cancelBtn.click();
        }
    }

    // Ctrl/Cmd + S to submit forms
    if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        const form = document.querySelector('form');
        if (form) {
            e.preventDefault();
            form.submit();
        }
    }
});
