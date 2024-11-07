
## Review Inventory
### List
```
ansible-inventory -i inventory.ini --list
```
### Ping

```
ansible myhosts -m ping -i ansible/inventory.ini
```

### Deploy
```
   ansible-playbook -i ansible/inventory.ini ansible/playbooks/deploy_quad_idle.yml
   ```