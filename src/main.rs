mod config;

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use log::{info, error};

// 初始化日志配置，这里简单设置为输出到标准输出，实际应用中可根据需求配置更复杂的日志格式和输出目标
fn init_logging() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
}

fn main() {
    init_logging();
    let configs = config::parse_config();
    info!("配置文件: {:?}", configs);

    // 创建一个通道，用于接收文件系统事件
    let (tx, rx) = channel();

    // 创建一个推荐的文件系统监视器
    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, Config::default().with_poll_interval(Duration::from_secs(10))).unwrap();

    // 要监控的文件夹路径，这里假设监控当前目录下的一个名为 "target_folder" 的文件夹，你可以根据实际需求修改路径
    let path = Path::new("/home/stone/resource/dailyNotes");
    let path2 = Path::new("/home/stone/resource");

    // 将文件夹添加到监视器中，使用递归模式，即监控文件夹及其子文件夹中的文件变化
    if let Err(e) = watcher.watch(path, RecursiveMode::Recursive) {
        error!("无法添加文件夹到监控列表: {:?}, path: {}", e.kind, &path2.display());
        return;
    }
    info!("开始监控文件夹: {}", &path.display());
    if let Err(e) = watcher.watch(path2, RecursiveMode::Recursive) {
        error!("无法添加文件夹到监控列表: {:?}, path: {}", e.kind, &path2.display());
        return;
    }
    info!("开始监控文件夹: {}", &path2.display());


    // 循环接收文件系统事件
    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    Ok(event) => {
                        // info!("文件系统事件: {:?}", event);
                        match event.kind {
                            EventKind::Create(kind) => {
                                info!("创建文件: {:?}", kind);
                            }
                            EventKind::Modify(kind) => {
                                info!("修改文件: {:?}", kind);
                            }
                            EventKind::Remove(kind) => {
                                info!("删除文件: {:?}", kind);
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        error!("处理文件系统事件时出错: {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("接收文件系统事件时出错: {:?}", e);
            }
        }
    }
}

