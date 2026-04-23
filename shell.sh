#!/bin/bash

# The Fuck Shell Integration Script

# Detect shell
SHELL_NAME=$(basename "$SHELL")

case "$SHELL_NAME" in
    bash)
        echo "Adding The Fuck to bash..."
        echo 'eval "$(thefuck --alias)"' >> ~/.bashrc
        echo 'alias fuck="sudo TF_HISTORY=$(fc -l -1 | sed "s/^[ ]*[0-9]*[ ]*//") THEFUCK_COMMAND_HISTORY=$TF_HISTORY thefuck"' >> ~/.bashrc
        ;;
    zsh)
        echo "Adding The Fuck to zsh..."
        echo 'eval "$(thefuck --alias)"' >> ~/.zshrc
        echo 'alias fuck="sudo TF_HISTORY=$(fc -l -1 | sed "s/^[ ]*[0-9]*[ ]*//") THEFUCK_COMMAND_HISTORY=$TF_HISTORY thefuck"' >> ~/.zshrc
        ;;
    fish)
        echo "Adding The Fuck to fish..."
        echo 'thefuck --alias | source' >> ~/.config/fish/config.fish
        echo 'alias fuck "sudo TF_HISTORY=(history --max 1 | string match -r " \\S+$") THEFUCK_COMMAND_HISTORY=$TF_HISTORY thefuck"' >> ~/.config/fish/config.fish
        ;;
    *)
        echo "Unsupported shell: $SHELL_NAME"
        echo "Please add the following to your shell configuration:"
        echo ""
        echo 'eval "$(thefuck --alias)"'
        exit 1
        ;;
esac

echo "The Fuck has been added to your $SHELL_NAME configuration!"
echo ""
echo "Please restart your shell or run:"
echo "  source ~/.bashrc    # for bash"
echo "  source ~/.zshrc     # for zsh"
echo "  fish                # for fish"
echo ""
echo "Then try:"
echo "  gti status"
echo "  fuck"