import re

with open('src/main.rs', 'r') as f:
    content = f.read()

# Collapse the nested if in main.rs
old_block = """            if event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) {
                if let Ok(Event::Key(key)) = event::read() {
                    if tx_event.send(Action::Key(key)).is_err() {
                        break;
                    }
                }
            }"""

new_block = """            if event::poll(std::time::Duration::from_millis(100)).unwrap_or(false)
                && let Ok(Event::Key(key)) = event::read()
            {
                if tx_event.send(Action::Key(key)).is_err() {
                    break;
                }
            }"""

# Note: The above might not match exactly due to whitespace. 
# Let's try a simpler approach if it doesn't work.
