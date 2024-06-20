package com.okx.zkcommitmobile.ui

import androidx.compose.foundation.background
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.toRoute
import com.okx.zkcommitmobile.DepositViewModel
import soup.compose.material.motion.animation.materialSharedAxisXIn
import soup.compose.material.motion.animation.materialSharedAxisXOut
import soup.compose.material.motion.animation.rememberSlideDistance

@Composable
fun DepositApp(modifier: Modifier = Modifier) {
    val navController = rememberNavController()
    val slideDistance = rememberSlideDistance()
    NavHost(
        navController = navController,
        modifier = modifier.background(MaterialTheme.colorScheme.background),
        startDestination = DepositScreen,
        enterTransition = { materialSharedAxisXIn(forward = true, slideDistance = slideDistance) },
        exitTransition = { materialSharedAxisXOut(forward = true, slideDistance = slideDistance) },
        popEnterTransition = {
            materialSharedAxisXIn(forward = false, slideDistance = slideDistance)
        },
        popExitTransition = {
            materialSharedAxisXOut(forward = false, slideDistance = slideDistance)
        }
    ) {
        composable<DepositScreen> {
            val viewModel = viewModel<DepositViewModel>()
            DepositScreen(
                deposits = viewModel.deposits.map { it.deposit },
                messages = viewModel.messages,
                onCreateDeposit = { navController.navigate(CreateDepositScreen) },
                onClaim = { navController.navigate(ClaimListScreen(id = it.id)) }
            )
        }

        composable<CreateDepositScreen> {
            val previousBackStackEntry = remember(it) { navController.previousBackStackEntry }
            val viewModel = viewModel<DepositViewModel>(previousBackStackEntry!!)
            CreateDepositScreen(
                onNavigateUp = { navController.navigateUp() },
                onCreate = { deposit ->
                    viewModel.createDeposit(deposit)
                    navController.popBackStack()
                }
            )
        }

        composable<ClaimListScreen> { backStackEntry ->
            val previousBackStackEntry =
                remember(backStackEntry) { navController.previousBackStackEntry }
            val viewModel = viewModel<DepositViewModel>(previousBackStackEntry!!)
            val id = backStackEntry.toRoute<ClaimListScreen>().id
            ClaimListScreen(
                deposit = viewModel.deposits.map { it.deposit }.first { it.id == id },
                messages = viewModel.messages,
                onNavigateUp = { navController.navigateUp() },
                onClaim = { deposit, index, _ -> viewModel.claim(deposit.id, index) }
            )
        }
    }
}
