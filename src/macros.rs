#[macro_export]
macro_rules! alias {
    ($state:ident, $($member:ident),*) => {
        $(let $member = &mut $state.$member;)*
    };

    ($app:ident.$state:ident, $($member:ident),*) => {
        $(let $member = &mut $app.$state.$member;)*
    }
}