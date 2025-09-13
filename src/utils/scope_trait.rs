use bevy::tasks::Scope as BevyScope;
use wacky_bag::traits::scope::{Scope, ScopeCreator, ScopeUser};

pub struct ComputeTaskPoolScope<'scope, 'env: 'scope, T>(&'scope BevyScope<'scope, 'env, T>);

impl<'scope, 'env: 'scope, ScopeFuncOutput> wacky_bag::traits::scope::Scope<'scope, ScopeFuncOutput>
    for ComputeTaskPoolScope<'scope, 'env, ScopeFuncOutput>
where
    ScopeFuncOutput: Send,
{
    fn spawn<F>(&'scope self, f: F) -> ()
    where
        F: FnOnce() -> ScopeFuncOutput + std::marker::Send + 'scope,
    {
        self.0.spawn(async move { f() });
    }
}

pub struct ComputeTaskPoolScopeCreator;

impl ScopeCreator for ComputeTaskPoolScopeCreator {

    type Output<'env, F>
        = Vec<<F as ScopeUser<'env>>::ScopeFnOutput>
    where
        F: wacky_bag::traits::scope::ScopeUser<'env> + 'env;

    fn scope<'env, F>(&mut self, f: F) -> Self::Output<'env, F>
    where
        F: wacky_bag::traits::scope::ScopeUser<'env> + 'env,
    {
        bevy::tasks::ComputeTaskPool::get().scope(move |s:&BevyScope<'_,'env, _ >|{
            let ntscope=ComputeTaskPoolScope(s);
            let extended_ntscope: &'env ComputeTaskPoolScope<'_, 'env, <F as ScopeUser<'env>>::ScopeFnOutput> =
                unsafe { std::mem::transmute(&ntscope) };
            f.use_scope(extended_ntscope);
        })
    }
}
