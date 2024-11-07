#!/bin/bash

# If --fast is passed, use the fast deploy playbook
if [[ "$1" == "--fast" ]]; then
    ansible-playbook -i ansible/inventory.ini ansible/playbooks/fast_deploy_quad_idle.yml
else
    ansible-playbook -i ansible/inventory.ini ansible/playbooks/deploy_quad_idle.yml
fi
