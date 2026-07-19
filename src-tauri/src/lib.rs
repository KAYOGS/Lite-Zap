use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Pega a janela principal do Lite-Zap
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval(r#"
                    (function() {
                        // 1. Bloqueia envio de logs de telemetria e erros para a Meta
                        window.onerror = function() { return true; };
                        window.onunhandledrejection = function() { return true; };
                        Object.defineProperty(navigator, 'webdriver', { get: () => undefined });

                        // 2. Filtro agressivo para deletar banners de download
                        const removerBannersChatos = () => {
                            // Seletores conhecidos e classes dinâmicas que o WhatsApp Web usa para o aviso
                            const seletores = [
                                '._ak6f', 
                                '[data-testid="download-app-banner"]',
                                'a[href*="://whatsapp.com"]'
                            ];

                            seletores.forEach(seletor => {
                                const elementos = document.querySelectorAll(seletor);
                                elementos.forEach(el => {
                                    // Se o elemento contiver textos sobre baixar o app, ele é deletado
                                    if (el.innerText && (
                                        el.innerText.includes('app') || 
                                        el.innerText.includes('Baixar') || 
                                        el.innerText.includes('Mac') || 
                                        el.innerText.includes('Windows') ||
                                        el.innerText.includes('Download')
                                    )) {
                                        el.remove();
                                    }
                                });
                            });
                        };

                        // Roda o filtro a cada 1 segundo para garantir que o banner nunca apareça
                        setInterval(removerBannersChatos, 1000);
                    })();
                "#);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
