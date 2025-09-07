use crate::DicomListener;
use crate::config::Config;
use crate::route::Route;
use std::path::PathBuf;

#[derive(Debug)]
pub struct EndpointManager {
    listeners: Vec<DicomListener>,
    routes: Vec<Route>,
}

impl From<crate::config::Config> for EndpointManager {
    fn from(config: Config) -> Self {
        let listeners = config
            .listeners
            .iter()
            .map(|listener| DicomListener::from(listener))
            .collect::<Vec<_>>();

        let mut routes = vec![];
        for cr in &config.routes {
            let listener = listeners.iter().find(|l| l.name() == cr.name);
            if listener.is_none() {
                continue;
            }
            let listener = listener.unwrap();
            let dir = PathBuf::from(match listener {
                DicomListener::Dcmtk(listener) => &listener.output,
            });
            let endpoints = config
                .endpoints
                .iter()
                .filter(|e| {
                    let tname = match e {
                        crate::config::Endpoint::Dicom(endpoint) => &endpoint.name,
                        crate::config::Endpoint::Dir(endpoint) => &endpoint.name,
                    };
                    cr.endpoints.contains(tname)
                })
                .collect::<Vec<_>>();
            let endpoints = endpoints
                .iter()
                .map(|endpoint| {
                    let te = (*endpoint).clone();
                    crate::endpoint::Endpoint::from(te)
                })
                .collect::<Vec<crate::endpoint::Endpoint>>();

            let route = crate::route::Route { dir, endpoints };
            routes.push(route);
        }

        Self { listeners, routes }
    }
}
