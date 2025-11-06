/// Конфигурация сетевого интерфейса с помощью команды ifconfig
pub fn ifconfig(
    adapter: &str,
    ip: impl AsRef<str>,
    mask: impl AsRef<str>,
    gw: impl AsRef<str>,
) -> Vec<String> {
    let cmd1 = format!(
        "ifconfig {adapter} {} netmask {}",
        ip.as_ref(),
        mask.as_ref(),
    );
    let cmd2 = format!("route add default gw {}", gw.as_ref());
    vec![cmd1, cmd2]
}
